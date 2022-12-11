pub fn primzahlgenerator(ende:u32) -> Vec<u32> {

    let mut primzahlenkette:Vec<u32> = vec![];

    for zahl in 2..1117{                                 // primzahl Nr. 187: 1117
        let mut primzahl: bool = true;                         // Annahme: Jede Zahl ist eine Primzahl
        for divisor in 2..zahl {                         // teile mit Zahlen bis zu der Zahl die untersucht wird
            if zahl % divisor == 0 {                           //wenn die Zahl teilbar ist:
                primzahl = false;                              // keine Primzahl
                break                                          // muss nicht weiter untersucht werden
            } 
        }

        if primzahl ==true {                                   // wenn Primzahl
            primzahlenkette.push(zahl);                        // füge in Primzahlen Zahlenette hinein
        }
        let ende: usize = ende as usize;                       //.len nur mit usize
        if primzahlenkette.len() == ende  {                    // nur bis zur angegebenen Anzahl Primzahlen suchen
            break                                              
        }
    }
    primzahlenkette
}