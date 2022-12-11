fn main(){

    let mut expotenzialkette: Vec<u128> = vec![1;6];                        // um anstieg auszubremsen initialisierung mit 6x 1
        for x in 1..100{                                                    // x = 0 überspringen 
            let x: f64 = x as f64;                                          // konvertierung in f32 da powf kein u
            let x = f64::powf(1.57,x);                                      // 1,57^a um die Zahl klein zu halten
            expotenzialkette.push(x as u128);                               // in expotenzialkette einfügen
        }
        
    println!("expotenzialkette = {:?}",expotenzialkette);
}