//mod libs;

use std::collections::HashMap;

fn table_rs() {
//
//
//    let mut achat = libs::mesPayements::Payment::with_name(5000);
//
//    achat.debiter(500);
//
//    println!("la nouvelle valeur est {:?}", achat.getMontant());
//
//    achat.crediter(1500);
//
//    println!("la nouvelle valeur est {:?}", achat.getMontant());
//
//    achat.doPayment(50000, libs::mesPayements::ModePaiement::Debit);
//
//    println!("la nouvelle valeur est {:?}", achat.getMontant());

    let numbers = [1.6, 2.0];
    println!("la liste des nombre {:?}", numbers);

    let tuple = (39, "ceci est just un test");
    println!("voici le contenu du tuple {:?}", tuple);

    let mut un_vecteur = Vec::new();
    un_vecteur.push(1);
    un_vecteur.push(2);

    let mut vec_macro = vec![1];
    vec_macro.push(2);

    //let _ = vec_macro.pop();

    let msg = if un_vecteur == vec_macro { "they're equal" } else { "nan! they look different to me" };
    println!("{} {:?} {:?} ", msg, un_vecteur, vec_macro);


    let mut fruits = HashMap::new();
    fruits.insert("pomme", 3);
    fruits.insert("mangue", 5);
    fruits.insert("orange", 6);
    fruits.insert("avocat", 9);
    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    let mut values: [u8; 4] = [1, 2, 3, 4];
    {
        let  all: &[u8] =&values[..];
        println!("All of them: {:?}", all);
    }
    {
        let les_deux = &mut  values[1..];
        les_deux[0] = 100;
        les_deux[1] = 99;

    }
    values[2] = 199;

    println!("les valeurs sont {:?} ", values);
}
