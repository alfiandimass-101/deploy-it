fn main() {
    let inventaris_item1: (i32, f64, u32) = (1, 15., 12);
    let inventaris_item2: (i32, f64, u32) = (2, 16., 13);
    let inventaris_item3: (i32, f64, u32) = (3, 17., 14);
    let inventaris_item4: (i32, f64, u32) = (1, 18., 19);
    let inventaris: [(i32, f64, u32); 4] = [
        inventaris_item1,
        inventaris_item2,
        inventaris_item3,
        inventaris_item4
    ];
}