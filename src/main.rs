

// ownership example

struct Book{
    pages: i32,
    rating: i32,
}

fn display_page_count(book:&Book){

    println!("Pages = {:?}",book.pages);

}

fn display_rating(book:&Book){
println!("Pages = {:?}",book.rating);

}





// activity oweneship 11
struct GroceryItem{
    id:i32,
    quantity :i32,

}

fn quanitity_item(grocey_item:&GroceryItem){

    println!("{:?}",grocey_item.quantity);


}

fn id_number(grocey_item:&GroceryItem){
    println!("{:?}",grocey_item.id);

}


// example using impl


struct Temprature{

    degrees_f : f64,

}

impl Temprature {
    fn show_temp(&self){
        println!("dgeree is {:?}",self.degrees_f);
    }
    fn cold_temp()->Self{
        Self { degrees_f: 22.3, }
    }

    fn boiling_temp()->Self{
        Self { degrees_f: 199.9, }
    }
    
}





fn main() {


    let book = Book{
        pages:10,
        rating:4
    };
    display_page_count(&book);
    display_rating(&book);


    let grocey_item = GroceryItem{
        id:10,
        quantity:4
    };
    id_number(&grocey_item);
    quanitity_item(&grocey_item);


    // using impl

    let temp = Temprature{degrees_f:99.9};
    temp.show_temp();

    let cold= Temprature::cold_temp();
    cold.show_temp(); 

    let boiling = Temprature::boiling_temp();
    boiling.show_temp();





}
