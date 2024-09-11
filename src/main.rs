mod cesar_encrypt;
mod disposable_mask;

use cesar_encrypt::scenario_cesar;
use disposable_mask::scenario_disposable_mask;


fn main() {
    // Call the scenario function 3 times
    scenario_cesar();
    scenario_cesar();
    scenario_cesar();

    scenario_disposable_mask();
    scenario_disposable_mask();
    scenario_disposable_mask();

}
