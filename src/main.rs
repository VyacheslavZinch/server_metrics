
mod sys_metrics;
use crate::sys_metrics::sys_metrics::*;
/*
        /cpu – возвращает информацию об использовании CPU;
        /memory – показывает состояние оперативной памяти;
        /network – отображает сетевую активность;
        /status – возвращает все данные сразу.
*/

fn main() {
    let data = MemInfo::new();
    data.json_data();
}
