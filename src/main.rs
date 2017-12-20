mod Items;


fn main() {
  let advancement_items = Items::getAdvancementItems();
  println!("advancement_items: {:?}", advancement_items);

  let nice_items = Items::getNiceItems();
  println!("nice_items: {:?}", nice_items);

  let junk_items = Items::getItemPool();
  println!("item_pool: {:?}", junk_items);

  let dungeon_items = Items::getDungeonPool();
  println!("dungeon_items: {:?}", dungeon_items);
}
