fn main() {
    println!("Hello, world!");
     let student : (f64,u32,char) =(4.1,12,'A');
     println!("{}",student.0);
     println!("{}",student.1);
     println!("{}",student.2);
     println!("Destructure");
     let (x,y,z)= student;
     println!("{}",x);
     println!("{}",y);
     println!("{}",z);

    let lottery:[u32;5] = [1,2,3,45,6];
    println!("{:#?}",lottery);
}
