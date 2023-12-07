#[macro_export]
macro_rules! solve {
    (
        $files:ident
        $(<< $name:literal $file:literal)*
        $question:literal
        $($answer:tt)+
    ) => {
        use ::lib::FilesExt;
        pub async fn solve() -> Result<::lib::Solution, String> {
            let $files = ::std::collections::BTreeMap::from_iter(vec![
                $((
                    $name.to_string(),
                    $file.to_string()
                )),*
            ]);
            Ok(
                ::lib::Solution {
                    files: $files.clone(),
                    question: $question.to_string(),
                    answer: format!($($answer)*)
                }
            )
        }
    };
}
