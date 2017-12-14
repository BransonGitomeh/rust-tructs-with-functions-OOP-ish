#[derive(Debug)]
struct Dob {
    dob: &'static str,
    mob: &'static str,
    yob: &'static str,
}

#[derive(Debug)]
struct Contacts {
    email: &'static str,
    phone_number: &'static str,
    mail: &'static str,
}

#[derive(Debug)]
struct Location {
    longitude: &'static str,
    latitude: &'static str,
    time: &'static str,
}

#[derive(Debug)]
struct Names {
    first_name: &'static str,
    last_name: &'static str,
    middle_name: &'static str,
}

#[derive(Debug)]
struct VerificationImages {
    front_id_url: &'static str,
    back_id_url: &'static str,
    user_selfie_id_url: &'static str,
}

#[derive(Debug)]
struct Account {
    id: &'static str,
}


impl Account {
    pub fn create(id: &'static str) -> Account {
        // call the db using async and user will have to await to resolve to log
        Account { id }
    }

    pub fn names(&mut self) -> Names {
        // call the db using async and user will have to await to resolve to log
        Names {
            first_name: "Branson",
            last_name: "Branson",
            middle_name: "Branson",
        }
    }

    pub fn contacts(&mut self) -> Contacts {
        // call the db using async and user will have to await to resolve to log
        Contacts {
            email: "Branson",
            phone_number: "Branson",
            mail: "Branson",
        }
    }

    pub fn dob(&mut self) -> Dob {
        // call the db using async and user will have to await to resolve to log
        Dob {
            mob: "Branson",
            yob: "Branson",
            dob: "Branson",
        }
    }

    pub fn last_location(&mut self) -> Location {
        // call the db using async and user will have to await to resolve to log
        Location {
            longitude: "Branson",
            latitude: "Branson",
            time: "Branson",
        }
    }

    pub fn other_locations(&mut self) -> Vec<Location> {
        let mut locations = Vec::new();
        let x = Location {
            longitude: "Branson",
            latitude: "Branson",
            time: "Branson",
        };

        locations.push(x);
        locations
    }

    pub fn verification_images(&mut self) -> Vec<VerificationImages> {
        let mut images = Vec::new();
        let x = VerificationImages {
            front_id_url: "Branson",
            back_id_url: "Branson",
            user_selfie_id_url: "Branson",
        };

        images.push(x);
        images
    }
}

fn main() {
    let mut new_account = Account::create("5744713810313216");

    println!("{:#?}", &new_account.verification_images());
    println!("{:#?}", &new_account.names());
    println!("{:#?}", &new_account.contacts());
    println!("{:#?}", &new_account.last_location());
    println!("{:#?}", &new_account.other_locations());
    println!("{:#?}", &new_account.dob());
}
