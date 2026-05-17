use clap::Parser;

#[derive(Parser)]
#[command(name = "rime-wubi")]
#[command(about = "五笔输入法词库管理工具", long_about = None)]
pub enum Command {
    #[command(about = "生成 Rime 词库配置文件")]
    Generate,

    #[command(about = "反查词语的五笔编码")]
    Lookup {
        #[arg(help = "要查询的词语")]
        phrase: String,
    },

    #[command(about = "添加词语到 custom.txt")]
    Add {
        #[arg(help = "要添加的词语")]
        phrase: String,
    },
}
