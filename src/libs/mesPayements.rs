 #[derive(Debug)]
 pub enum ModePaiement {
     Debit,
     Credit,
     Paypal
 }

 #[derive(Debug)]
 pub struct Payment {
     mode: ModePaiement,
     montant: u64
 }
 fn paiement_dedit(arg: u64){
    println!("processus pour debiter la carte {}", arg);
 }
 fn paiement_credit(arg: u64){
    println!("processus pour crediter la carte {}", arg);
 }
 fn paiement_redirect(arg: u64){
    println!("processus pour rediriger le paiement de  la carte {}", arg);
 }
 
 impl ModePaiement {
     fn payer(&self, somme: u64){
         match self {
             ModePaiement::Debit => paiement_dedit(somme),
             ModePaiement::Credit => paiement_credit(somme),
             ModePaiement::Paypal => paiement_redirect(somme)
             //None => println!("notre banque ne gere pas ce cas la. Merci!"),
         }
     }
 }

 impl Payment {
    pub fn with_name(valeur_init: u64) -> Payment {
        Payment{
            mode: ModePaiement::Credit, 
            montant: valeur_init
        }
     }
     pub fn doPayment(&mut self, somme: u64, mode: ModePaiement){
        match mode {
            ModePaiement::Debit => self.debiter(somme),
            ModePaiement::Credit => self.crediter(somme),
            ModePaiement::Paypal => self.Paypal(somme)
            //None => println!("notre banque ne gere pas ce cas la. Merci!"),
        }
     }
     pub fn crediter(&mut self, somme: u64){
         println!("reception d'un credit de {}fr", somme);
         self.setMontant(self.getMontant()+somme);
     }

    pub fn debiter(&mut self, somme: u64){
        println!("reception d'un debit de {}fr", somme);
        self.setMontant(self.getMontant()-somme);
    }

    fn Paypal(&mut self, somme: u64){
        println!("reception d'un paiement par paypal de {}fr", somme);
        self.setMontant(self.getMontant()-somme);
    }
     fn setMontant(&mut self, valeur: u64){
         self.montant = valeur;
     }

    pub fn getMontant(&self) -> u64 {
        self.montant
     }
 }


 fn save_pay_mode() -> ModePaiement{
    ModePaiement::Credit
 }


