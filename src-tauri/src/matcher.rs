use crate::grpc::proto::ProfileLibrary;


pub fn match_lib(libraries: Vec<ProfileLibrary>) {
    let test = libraries.into_iter().filter(|lib| lib.rules.iter().for_each(|element| {
        if element.action == "allow" {
            return true;
        }
        else {
            return false;
        }}));
}