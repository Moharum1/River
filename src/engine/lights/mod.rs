pub(crate) trait Light{
    fn preprocess(&self);
}


pub struct GeneralLight{

}

impl Light for GeneralLight{
    fn preprocess(&self) {

    }
}