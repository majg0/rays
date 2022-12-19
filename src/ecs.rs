use std::ops::{Index, IndexMut};

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/******************************************************************************
 * Handle
 *****************************************************************************/

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Handle {
    pub g: u32,
    pub i: u32,
}

impl Handle {
    pub const INVALID: Handle = Handle {
        g: u32::MAX,
        i: u32::MAX,
    };

    pub fn new(g: u32, i: u32) -> Handle {
        Handle { g, i }
    }
}

/******************************************************************************
 * ComponentData
 *****************************************************************************/

#[derive(Debug, Default)]
pub struct ComponentData<C> {
    pub c: AtomicRefCell<Vec<C>>,
}

impl<C: Clone> Clone for ComponentData<C> {
    fn clone(&self) -> Self {
        Self {
            c: AtomicRefCell::new((*self.c.borrow()).clone()),
        }
    }
}

impl<C> ComponentData<C> {
    pub fn new() -> ComponentData<C> {
        ComponentData {
            c: AtomicRefCell::new(Vec::new()),
        }
    }
}

impl<C: Serialize> Serialize for ComponentData<C> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.c.borrow()).serialize(s)
    }
}

impl<'a, C: Deserialize<'a>> Deserialize<'a> for ComponentData<C> {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        Ok(ComponentData {
            c: AtomicRefCell::new(Deserialize::deserialize(d)?),
        })
    }
}

/******************************************************************************
 * EntityStorage
 *****************************************************************************/

pub trait EntityStorage {
    fn len(&self) -> u32;
    fn is_empty(&self) -> bool;
    fn entities(&self) -> Box<dyn Iterator<Item = Handle> + '_>;
    fn contains(&self, e: Handle) -> bool;
}

/******************************************************************************
 * ComponentIndex
 *****************************************************************************/

#[derive(Debug, Default, Clone)]
pub struct ComponentIndex {
    /// dense
    pub c2e: Vec<Handle>,
    /// sparse
    pub e2c: Vec<Handle>,
}

impl ComponentIndex {
    pub fn new() -> ComponentIndex {
        ComponentIndex {
            c2e: Vec::new(),
            e2c: Vec::new(),
        }
    }

    pub fn contains(&self, e: Handle) -> bool {
        (e.i as usize) < self.e2c.len() && self.e2c[e.i as usize].g == e.g
    }
}

impl Serialize for ComponentIndex {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.c2e.serialize(s)
    }
}

impl<'a> Deserialize<'a> for ComponentIndex {
    fn deserialize<D: Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
        let c2e: Vec<Handle> = Deserialize::deserialize(d)?;

        let mut e2c = Vec::new();
        for (i, e) in c2e.iter().enumerate() {
            if e.i as usize >= e2c.len() {
                e2c.resize(e.i as usize + 1, Handle::INVALID);
            }
            e2c[e.i as usize] = Handle::new(e.g, i as u32);
        }

        Ok(ComponentIndex { c2e, e2c })
    }
}

/******************************************************************************
 * Read / Write
 *****************************************************************************/

pub struct ReadStorage<'a, C> {
    pub c: AtomicRef<'a, Vec<C>>,
    index: &'a ComponentIndex,
}

impl<'a, C> Index<usize> for ReadStorage<'a, C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.c[self.index.e2c[index].i as usize]
    }
}

pub struct WriteStorage<'a, C> {
    c: AtomicRefMut<'a, Vec<C>>,
    index: &'a ComponentIndex,
}

impl<'a, C> Index<usize> for WriteStorage<'a, C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.c[self.index.e2c[index].i as usize]
    }
}

impl<'a, C> IndexMut<usize> for WriteStorage<'a, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.c[self.index.e2c[index].i as usize]
    }
}

pub struct InsertStorage<'a, C> {
    c: AtomicRefMut<'a, Vec<C>>,
    index: &'a mut ComponentIndex,
}

impl<'a, C> InsertStorage<'a, C> {
    pub fn insert(&mut self, e: Handle, c: C) {
        if (e.i as usize) >= self.index.e2c.len() {
            self.index.e2c.resize(e.i as usize + 1, Handle::INVALID);
        }

        self.index.e2c[e.i as usize] = Handle::new(e.g, self.index.c2e.len() as u32);
        self.index.c2e.push(e);
        self.c.push(c);
    }
}

/******************************************************************************
 * ComponentStorage
 *****************************************************************************/

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ComponentStorage<C> {
    pub data: ComponentData<C>,
    pub index: ComponentIndex,
}

impl<C> ComponentStorage<C> {
    pub fn new() -> ComponentStorage<C> {
        ComponentStorage {
            data: ComponentData::new(),
            index: ComponentIndex::new(),
        }
    }

    pub fn read(&self) -> ReadStorage<'_, C> {
        ReadStorage {
            c: self.data.c.borrow(),
            index: &self.index,
        }
    }

    pub fn write(&self) -> WriteStorage<'_, C> {
        WriteStorage {
            c: self.data.c.borrow_mut(),
            index: &self.index,
        }
    }

    pub fn insert(&mut self) -> InsertStorage<'_, C> {
        InsertStorage {
            c: self.data.c.borrow_mut(),
            index: &mut self.index,
        }
    }

    //     pub fn set(&mut self, e: Handle, c: C) {
    //         let mut data = self.data.c.borrow_mut();
    //         match self.index.prep_set(e) {
    //             PrepAddResult::Mutate(i) => {
    //                 data[i] = c;
    //             }
    //             PrepAddResult::Append => {
    //                 data.push(c);
    //             }
    //         }
    //     }

    // fn remove(&mut self, _e: Handle) {
    //        if e.i as usize >= self.index.e2c.len() {
    //            self.index.e2c.resize(e.i as usize + 1, Handle::INVALID);
    //            return;
    //        }
    //
    //        let ch = self.index.e2c[e.i as usize];
    //        self.index.e2c[e.i as usize] = Handle::INVALID;
    //
    //        if ch.g != e.g {
    //            return;
    //        }
    //
    //        let e = self.data.e.pop().unwrap();
    //        let c = self.data.c.pop().unwrap();
    //        if (ch.i as usize) < self.data.e.len() {
    //            self.data.e[ch.i as usize] = e;
    //            self.data.c[ch.i as usize] = c;
    //            self.index.e2c[e.i as usize] = ch;
    //        }
    // }
}

impl<C> EntityStorage for ComponentStorage<C> {
    fn len(&self) -> u32 {
        self.index.c2e.len() as u32
    }

    fn is_empty(&self) -> bool {
        self.index.c2e.is_empty()
    }

    fn entities(&self) -> Box<dyn Iterator<Item = Handle> + '_> {
        Box::new(self.index.c2e.iter().copied())
    }

    fn contains(&self, e: Handle) -> bool {
        self.index.contains(e)
    }
}

/******************************************************************************
 * Allocator
 *****************************************************************************/

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Allocator {
    gen: Vec<u32>,
    dead: Vec<u32>,
}

impl Allocator {
    pub fn new() -> Allocator {
        Allocator {
            gen: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> Handle {
        let i = self.dead.pop().unwrap_or_else(|| {
            let i = self.gen.len() as u32;
            self.gen.push(0);
            i
        });

        Handle {
            g: self.gen[i as usize],
            i,
        }
    }

    pub fn dealloc(&mut self, h: Handle) {
        if !self.contains(h) {
            return;
        }
        self.gen[h.i as usize] = h.g + 1;
        self.dead.push(h.i);
    }
}

impl EntityStorage for Allocator {
    fn len(&self) -> u32 {
        self.gen.len() as u32
    }

    fn is_empty(&self) -> bool {
        self.gen.is_empty()
    }

    fn entities(&self) -> Box<dyn Iterator<Item = Handle> + '_> {
        Box::new(
            self.gen
                .iter()
                .enumerate()
                .map(|(i, g)| Handle { g: *g, i: i as u32 }),
        )
    }

    fn contains(&self, h: Handle) -> bool {
        self.gen[h.i as usize] == h.g
    }
}

/******************************************************************************
 * Utilities
 *****************************************************************************/

pub fn iterate<'a>(s: &'a mut [&'a dyn EntityStorage]) -> Box<dyn Iterator<Item = usize> + 'a> {
    s.sort_by_key(|a| a.len());
    Box::new(
        s[0].entities()
            .filter(|e| s[1..].iter().all(|s| s.contains(*e)))
            .map(|e| e.i as usize),
    )
}
