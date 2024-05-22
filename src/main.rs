struct Shuttle<'a> {
    name: &'a str,
}

/*In order to return the second input parameter, we need to assign it a lifetime and declare what reference the function will return;
in this case, we are wanting to return the msg parameter, so we are going to assign both it and the return value a lifetime of 'b.
Remember, when we assign a lifetime to a field in a struct, any functions that implement &self will in turn return the lifetime associated
with &self by default.  If you are wanting to return a reference value of another parameter, you are going to have to assign it a separate
lifetime.  You must also implement it in your impl block, as shown below:
*/

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmission message: {}", msg);
        return msg;
    }
}

fn main() {
    let vehicle = Shuttle { name: "Endeavor" };

    let sender = vehicle.send_transmission("Greetings from orbit!");

    println!("sender is {}", sender);
}
