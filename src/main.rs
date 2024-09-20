mod cesar_encrypt;
mod disposable_mask;
mod cesar_modified;

use cesar_encrypt::scenario_cesar;
use disposable_mask::scenario_disposable_mask;
use cesar_modified::scenario1_cesar_modified;
use cesar_modified::scenario2_cesar_modified;


fn main() {
    // Call the scenario function 3 times
    print!("cesar encryption : \n");
    scenario_cesar();
    scenario_cesar();
    scenario_cesar();
    print!("Eve pourrait facilement décoder le message, en effet il n’y a que 26-1 possibilité pour déchiffrer le message.\n");

    print!("disposable mask : \n");
    scenario_disposable_mask();
    scenario_disposable_mask();
    scenario_disposable_mask();
    print!("Eve ne peut pas déchiffrer ce message avec l’information qu’elle voit, car le chiffrement du masque jetable aussi appelé
    chiffre de Vernam est un chiffrement parfaitement sûr. Pour chaque bit du message, il y a 2 choix possibles pour le bit de la
    clé (0 ou 1). Donc, pour un message de longueur n, il y a 2 puissance n*16 clés possibles. \n");

    print!("cesar modified scenario 1: \n");
    scenario1_cesar_modified();
    scenario1_cesar_modified();
    scenario1_cesar_modified();
    print!("Eve ne peut pas déchiffrer ce message avec l’information qu’elle voit, car le chiffrement de César modifié est
    un chiffrement parfaitement sûr. Pour chaque bit du message, il y a 26 clés possibles. Donc, pour un message de longueur n,
    il y a 26 puissance n clés possibles. \n");
    print!("cesar modified scenario 2: \n");
    scenario2_cesar_modified();
    scenario2_cesar_modified();
    scenario2_cesar_modified();
    print!("Bob ne peut pas déchiffrer correctement le message car il a été modifié par Eve.\n
    Ce chiffrement est plus sécurisé que le chiffrement de César classique, mais il reste vulnérable à une attaque par modification
    de message. De plus pour un message de longueur n il y a 26 puissance n possibilité de clé ce qui est moins puissant que le masque
    jetable. \n");
}
