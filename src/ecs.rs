use std::{alloc::Layout, any::TypeId, collections::HashMap, ptr::NonNull};

pub struct Ecs {
    next_index: usize,
    component_stores: HashMap<TypeId, ComponentStore>,
}

impl Ecs {
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_index: 0,
            component_stores: HashMap::new(),
        }
    }

    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.next_index
    }

    pub fn insert<ED>(&mut self, entity_definition: ED) -> EntityIndex
    where
        ED: EntityDefinition,
    {
        let entity_index = self.allocate_index();
        entity_definition.store_component(self, entity_index.index);
        entity_index
    }

    fn allocate_index(&mut self) -> EntityIndex {
        // TODO check for reusable indices (from delete entities)
        let next_index = self.next_index;
        let index = EntityIndex {
            index: next_index,
            generation: 0,
        };
        self.next_index += 1;
        index
    }

    fn store_component<C>(&mut self, index: usize, component: C)
    where
        C: 'static,
    {
        let component_store = self
            .component_stores
            .entry(TypeId::of::<C>())
            .or_insert_with(|| {
                ComponentStore::new(std::alloc::Layout::new::<C>(), drop_component_fn::<C>)
            });

        component_store.store(index, component);
    }
}

impl Default for Ecs {
    fn default() -> Self {
        Self::new()
    }
}

pub trait EntityDefinition {
    fn store_component(self, ecs: &mut Ecs, index: usize);
}
impl<A: 'static> EntityDefinition for (A,) {
    fn store_component(self, ecs: &mut Ecs, index: usize) {
        ecs.store_component(index, self.0);
    }
}
impl<A: 'static, B: 'static> EntityDefinition for (A, B) {
    fn store_component(self, ecs: &mut Ecs, index: usize) {
        ecs.store_component(index, self.0);
        ecs.store_component(index, self.1);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityIndex {
    index: usize,
    generation: usize,
}

const BITSET_BIT_COUNT: usize = 65536;
const fn bitset_word_count<T>() -> usize {
    BITSET_BIT_COUNT / (std::mem::size_of::<T>() * 8)
}

struct ComponentStore {
    data: NonNull<u8>,
    layout: Layout,
    len: usize,
    reserved_len: usize,
    drop: unsafe fn(*mut u8),
    entities_bitset: [u64; bitset_word_count::<u64>()],
}

impl ComponentStore {
    pub fn new(component_layout: Layout, drop: unsafe fn(*mut u8)) -> Self {
        let reserved_len = if component_layout.size() == 0 {
            usize::MAX
        } else {
            0
        };

        Self {
            data: NonNull::dangling(),
            layout: component_layout,
            len: 0,
            reserved_len,
            drop,
            entities_bitset: [0u64; bitset_word_count::<u64>()],
        }
    }

    pub fn store<C>(&mut self, index: usize, mut component: C) {
        assert!(index < BITSET_BIT_COUNT, "ComponentStore is full");
        self.entities_bitset.set_bit(index);
        self.resize(index + 1);

        if self.layout.size() > 0 {
            // SAFETY:
            // The chunked of data has just been resized to ensure it can store the component
            unsafe { self.write(index, std::ptr::addr_of_mut!(component).cast()) };
        }
    }

    pub fn clear(&mut self) {
        let len = self.len;
        for i in 0..len {
            if self.entities_bitset.bit(i) {
                self.entities_bitset.unset_bit(i);
                unsafe {
                    let ptr = self.data.as_ptr().add(i * self.layout.size());
                    (self.drop)(ptr);
                }
                self.len -= 1;
            }
        }
    }

    fn resize(&mut self, len: usize) {
        self.reserve_exact(len);
        self.len = len;
    }

    fn reserve_exact(&mut self, len: usize) {
        if self.reserved_len >= len {
            return;
        }

        let new_reserved_len = len;
        let new_layout = array_layout(self.layout, new_reserved_len);

        // SAFETY:
        // - The layout is guaranteed to have a non-zero size because we don't reserve
        //   when using ZST
        let new_data = unsafe {
            if self.reserved_len == 0 {
                std::alloc::alloc(new_layout)
            } else {
                std::alloc::realloc(
                    self.data.as_ptr(),
                    array_layout(self.layout, self.reserved_len),
                    new_layout.size(),
                )
            }
        };

        self.reserved_len = new_reserved_len;
        self.data = NonNull::new(new_data).expect("ComponentStore allocation failed");
    }

    /// # Safety
    /// - index must be in the bounds of the allocated chunk of data
    unsafe fn write(&mut self, index: usize, data_ptr: *mut u8) {
        let dst_ptr = self.data.as_ptr().add(index);
        std::ptr::copy_nonoverlapping(data_ptr, dst_ptr, 1);
    }
}

impl Drop for ComponentStore {
    fn drop(&mut self) {
        if self.layout.size() == 0 {
            return;
        }

        self.clear();
        let layout = array_layout(self.layout, self.reserved_len);

        // SAFETY:
        // - self.data has been allocated with the same allocator
        // - the given layout is the same that the one that's been
        //   used to allocate the chunk of memory
        unsafe {
            std::alloc::dealloc(self.data.as_ptr(), layout);
        }
    }
}

unsafe fn drop_component_fn<T>(ptr: *mut u8) {
    ptr.cast::<T>().drop_in_place();
}

// TODO: Remove when std::alloc::Layout::array stabilizes
const fn array_layout(layout: Layout, len: usize) -> Layout {
    let array_size = layout.size() * len;
    assert!(layout.size() != 0 && len <= max_size_for_align(layout.align()) / layout.size());

    // SAFETY: layout being a valid layout
    // - layout.align() is non-zero
    // - layout.align() is a power of two
    // - We checked that when rounded up to the neared multiple of align, array_size doesn't overflow isize
    unsafe { Layout::from_size_align_unchecked(array_size, layout.align()) }
}

const fn max_size_for_align(align: usize) -> usize {
    isize::MAX as usize - (align - 1)
}

trait Bitset {
    fn set_bit(&mut self, nth: usize);
    fn unset_bit(&mut self, nth: usize);
    fn bit(&self, nth: usize) -> bool;
}

impl Bitset for [u64; 1024] {
    fn set_bit(&mut self, nth: usize) {
        let word_index = nth >> 6;
        let bit = nth & 63;
        self[word_index] |= 1 << bit;
    }

    fn unset_bit(&mut self, nth: usize) {
        let word_index = nth >> 6;
        let bit = nth & 63;
        self[word_index] &= !(1 << bit);
    }

    fn bit(&self, nth: usize) -> bool {
        let word_index = nth >> 6;
        let bit = nth & 63;
        (self[word_index] >> bit) & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecs_new() {
        let ecs = Ecs::new();
        assert_eq!(ecs.entity_count(), 0);
    }

    struct Player;
    struct Enemy;
    struct Health(i16);

    #[test]
    fn ecs_insert() {
        let mut ecs = Ecs::new();
        let entity_index = ecs.insert((Player, Health(10)));
        assert_eq!(entity_index.index, 0);
        assert_eq!(entity_index.generation, 0);

        let entity_index = ecs.insert((Enemy, Health(5)));
        assert_eq!(entity_index.index, 1);
        assert_eq!(entity_index.generation, 0);

        assert_eq!(ecs.entity_count(), 2);
    }

    #[test]
    fn bitset_set() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(5);
        assert_eq!(bitset[0], 32);

        bitset.set_bit(0);
        assert_eq!(bitset[0], 33);

        bitset.set_bit(64);
        assert_eq!(bitset[1], 1);
    }

    #[test]
    fn bitset_bit() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(5);
        assert!(bitset.bit(5));
        bitset.set_bit(65);
        assert!(bitset.bit(65));
    }

    #[test]
    fn bitset_unset() {
        let mut bitset = [0u64; 1024];
        bitset.set_bit(3);
        assert!(bitset.bit(3));
        bitset.unset_bit(3);
        assert!(!bitset.bit(3));
    }
}
