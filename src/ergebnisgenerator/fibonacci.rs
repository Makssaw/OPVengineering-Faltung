pub fn fibonacci(fibzahl:usize,fibonaccikette: &Vec<u128>) -> u128{

    let fibzahl_usize: usize = fibzahl as usize;                            // Vektorelemente müssen usize sein
    let fibzahl= fibonaccikette[fibzahl_usize-2]+fibonaccikette[fibzahl_usize-1];
    fibzahl
}


/*
fn fibonacci(fibzahl:u32) -> u32{                   // https://de.wikipedia.org/wiki/Fibonacci-Folge

    if fibzahl == 0 {                               // erste bedingung f(0) = 0
        return 0;
    }

    else if fibzahl == 1 {                          // zweite bedingung f(1) = 1
        return 1;
    }

    fibonacci(fibzahl-1)+fibonacci(fibzahl-2)       // Formel ab x > 2: f(x) = f(x-1) + f(x-2)
}
*/