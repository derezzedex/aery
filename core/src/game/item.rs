#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bitcode::Encode, bitcode::Decode)]
pub struct Item(pub usize);

impl TryFrom<i32> for Item {
    type Error = ();

    fn try_from(value: i32) -> Result<Item, Self::Error> {
        if value <= 0 {
            Err(())
        } else {
            Ok(Item(value as usize))
        }
    }
}

#[derive(Debug, Clone, Copy, bitcode::Encode, bitcode::Decode)]
pub struct Trinket(pub usize);

impl From<Trinket> for Item {
    fn from(val: Trinket) -> Self {
        Item(val.0)
    }
}

#[derive(Debug, Clone, Copy, bitcode::Encode, bitcode::Decode)]
pub struct Inventory {
    pub items: [Option<Item>; 6],
}

impl IntoIterator for Inventory {
    type Item = Option<Item>;
    type IntoIter = std::array::IntoIter<Self::Item, 6>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl From<[Option<Item>; 6]> for Inventory {
    fn from(items: [Option<Item>; 6]) -> Self {
        Self { items }
    }
}
