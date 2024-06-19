

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

// activity 12 impl


enum Color{
    Red,
    Blue,
}


impl Color {
    fn print(&self){
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
            
        }
    }
    
}

struct Dimention{
    x:f64,
    y:f64,
    z:f64,
}

impl Dimention {
    fn print(&self){
        println!("x is {:?}",self.x);
        println!("y is {:?}",self.y);
        println!("z is {:?}",self.z);
    }
}

struct ShippingBox{
    color:Color,
    dimention:Dimention,
    weight: f64,

}

impl  ShippingBox {
    fn new(color: Color, dimention: Dimention, weight: f64)->Self{
        Self { 
            color,
            dimention,
            weight
         }

    }


    fn print(&self){
        self.color.print();
        self.dimention.print();
        println!("weight:{:?}",self.weight);
    }
    


   

    
    
}




fn main() {


    let book = Book{
        pages:10,
        rating:4,
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


    // main activity 12


    let small_dimention = Dimention{
        x:2.0,
        y:3.0,
        z:4.0,
    };

    let my_box = ShippingBox::new(Color::Blue, small_dimention, 3.4);
    my_box.print();



    





}
