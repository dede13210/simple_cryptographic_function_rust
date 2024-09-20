mod cesar_encrypt;
mod disposable_mask;
mod cesar_modified;

use cesar_encrypt::scenario_cesar;
use disposable_mask::scenario_disposable_mask;
use cesar_modified::scenario_cesar_modified;


fn main() {
    // Call the scenario function 3 times
    // print!("cesar encryption : \n");
    // scenario_cesar();
    // scenario_cesar();
    // scenario_cesar();

    print!("disposable mask : \n");
    scenario_disposable_mask();
    scenario_disposable_mask();
    scenario_disposable_mask();

    // print!("cesar modified : \n");
    // scenario_cesar_modified();
    // scenario_cesar_modified();
    // scenario_cesar_modified();


}
