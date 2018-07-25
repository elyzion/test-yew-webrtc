use stdweb::Value;
use stdweb::Reference;

#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Navigator")]
pub struct Navigator( Reference );

pub fn navigator() -> Navigator {
    unsafe { js!( return navigator; ).into_reference_unchecked() }.unwrap()
}

impl Navigator {

    pub fn app_name(&self) -> String {
        "tmp".to_owned()
    }

}
