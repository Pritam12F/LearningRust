pub fn main2(){
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if(count >= 20){
                break 'inner1;
            }

            count+=2;
        }

        count+=5;

        'inner2: loop {
            if(count >= 30){
                break 'outer;
            }

            count+=5;
        }
    }

    assert_eq!(30, count);

    println!("Success");
}