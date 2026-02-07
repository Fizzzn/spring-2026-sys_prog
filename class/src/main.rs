fn get_number(y:i32) -> i32{
    let x =5;
    //return x+y
    
    //x+y

    let y = {
        let x = x + 3;
        y + x

    };
    y + x // if you only have y it'll return 18 but if you put 
    // x+y youll get 23
    //^dude you can just do that? no ;? cause its an expression???
    //i wonder why it refuses to run with ; (x+y;)
// you have to provide data type for the paramaters in the func.
}
//fn fahrenheit_to_celsius(f: f64) -> f64{
 //let celsius = (f - 32) * 0.556;
//}

//fn celsius_to_fahrenheit(c: f64) -> f64{
//let fahrenheit = c * 1.8 + 32;
//}

fn RGB_Selector( c: char) -> (u8,u8,u8){
//let GREEN tup = i32 (0,255,0);
//let RED tup = i32 (255,0,0);
//let BLUE tup = i32 (0,0,255);

//if c == RED{return (255,0,0);} // if chain is boring use MATCH
 //else if c == GREEN {return (0,255,0);}
   // else {return (0,0,255);}
    match c{ //this is signifucantly better then an if chain  btw
        'R' => (255,0,0),
        'G' => (0,255,0),
        'B' => (0,0,255),
        _ => (0,0,0),

    }

}


//const freezing_f: i32 = 32;
fn main() {
    println!("Hello, world!");
    let x =5;
    let y = 10;

    println!("THe value of x is: {}", x);

    println!("{}*2 = {}",x,y); //will it guess here?


    //mut keyword explicity indicates that a variable's value can change
    let mut result = 100; //when creating variable  you'll need the complier 
                    //to know if you are planning to change it
    result += 100;
    println!("{}",result);

    let mut example: f32 = 0.0;
        //example += 1.5;
    // in other programming languages 
    //data types get converted implicity
    // int x = 0;
    // f y = 1.5;
    // x+y
    //you need to provide data type essentially using like f32 (float)
    example += 1.0;
        println!("{}", example);

        let z = 10;
        let w = 2.0;

        //let multi = x * y;

            let multi = z* (w as i32);
            //you arnet changin the value of w just the data type
            //this can be done on the fly
                println!("{}",multi);

    //const MAX_POINTS: u32 = 100_000; // this is valid

   // const MAX_POINTS:i128 = 100_000_00000000_000000;
     //   let num = MAX_POINTS*10;
       // println!("The maximum points allowed: {}", num);

        //shadowing
    let number = "25"; // if you have a float here rust will complain that your feading it the wrong data type
    // it will tell you 
    let resulted: Result<i32,_> = number.parse();
    println!("{:?}", resulted);

   let f = 10;
    //f += 10; // f is immutable wont work, 
    // in rust name isnt a unique identifier

    let f = f + 10; //but this will work
    println!("this is f due to it being in scope and shadowing: {}",f);

    //fn main() {
    // Shadowing
    //let x = 5;
    //let x = x + 1;  // Creates a new variable
    
    // Mutation
   // let mut y = 5;
   // y = y + 1;  // Modifies the existing variable
    
   // println!("x: {}, y: {}", x, y);
//}
 //lets make funtions
 println!("{}",get_number(10));


 //in class assignment

 ///////////////////////////////

   // Creating a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring (pattern matching)
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    
    // Accessing tuple elements using dot notation
    println!("First: {}, Second: {}, Third: {}", tup.0, tup.1, tup.2);

    // Tuple as a return type
    let (product, sum) = calculate(3, 4);
    println!("Product: {}, Sum: {}", product, sum);

    let repeated = [3 ; 5]; //equevelent to [3,3,3,3,3]

    let index: i32 = 0; //usize

    let element = repeated[index as usize];

    println!("{}",element);

    //were going to asseot a letter like RGB
    //and we should return 
    // RED tuple (255,0,0)
    // GREEN tuple (0,255,0)
    // BLUE tuple (0,0,255)

    //wriete a funtion that asspets car 'r' G B 
    //and return above specified tuple
    
    //let res = RGB_Selector('R');
    //println!("{:?}",res);

    //LOOPS

    let letters = ['R','G','B'];

  //  for l in letters.iter(){
    //    let res = RGB_Selector('l');
      //  println!("{:?}",res);
  //  } 

     for idx in 0.. letters.len(){
        let res = RGB_Selector(letters[idx]);
        println!("{:?}",res);
    } 
}

fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x * y, x + y)



}
