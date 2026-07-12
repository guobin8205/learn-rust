// ===== 17.1 Trait 对象（dyn Trait）=====
// 用 cargo run --bin trait_objects 运行

// ========================================================
// 问题：如何把「不同类型」的实例放进同一个集合？
// ========================================================

// 场景：一个 GUI 库，有 Button、TextBox 等不同组件
// 它们都实现了 Draw trait，但具体类型不同
// 我们想把它们放进同一个 Vec 里统一管理

pub trait Draw {
    fn draw(&self);
}

// ---------- 不同的组件类型 ----------
pub struct Button {
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制按钮: [{}]", self.label);
    }
}

pub struct TextBox {
    pub text: String,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("绘制文本框: <{}>", self.text);
    }
}

pub struct Image {
    pub filename: String,
}

impl Draw for Image {
    fn draw(&self) {
        println!("绘制图片: 🖼️ {}", self.filename);
    }
}

// ========================================================
// ⭐ Screen：用 trait 对象存储不同类型
// ========================================================

pub struct Screen {
    // Box<dyn Draw> = 「实现了 Draw 的某种类型」的 trait 对象
    // dyn 关键字表示「动态分发」
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen { components: Vec::new() }
    }

    pub fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in &self.components {
            component.draw();   // 动态分发：运行时决定调用哪个 draw
        }
    }
}

fn main() {
    println!("===== 1. trait 对象：存储不同类型 =====\n");

    let mut screen = Screen::new();
    screen.add(Box::new(Button { label: String::from("确定") }));
    screen.add(Box::new(TextBox { text: String::from("Hello") }));
    screen.add(Box::new(Image { filename: String::from("logo.png") }));

    // ✅ 不同类型（Button/TextBox/Image）放进了同一个 Vec！
    screen.run();

    println!("\n===== 2. 静态分发 vs 动态分发 =====\n");

    // ---------- 静态分发（泛型 + trait bound）----------
    // 编译期为每种类型生成专用代码（单态化）
    fn static_draw<T: Draw>(item: &T) {
        item.draw();
    }
    let btn = Button { label: String::from("按钮") };
    let txt = TextBox { text: String::from("文本") };
    static_draw(&btn);   // 编译期生成 static_draw::<Button>
    static_draw(&txt);   // 编译期生成 static_draw::<TextBox>
    println!("静态分发：编译期生成专用代码，零成本");

    // ---------- 动态分发（trait 对象）----------
    // 运行时通过虚表（vtable）查找方法
    let items: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: String::from("按钮2") }),
        Box::new(TextBox { text: String::from("文本2") }),
    ];
    for item in &items {
        item.draw();   // 运行时查 vtable 决定调用哪个
    }
    println!("动态分发：运行时查虚表，有少量开销");

    println!("\n===== 3. 静态 vs 动态对比 =====\n");

    println!("| 特性        | 静态分发（泛型）       | 动态分发（dyn Trait）   |");
    println!("|-------------|-----------------------|------------------------|");
    println!("| 何时确定方法 | 编译期                 | 运行期                  |");
    println!("| 性能        | 最快（零成本）          | 稍慢（虚表查找）         |");
    println!("| 能否存混合类型 | ❌（Vec<T> 只能一种）| ✅（Vec<Box<dyn T>>）   |");
    println!("| 二进制大小   | 更大（每种类型一份代码）| 更小（共享一份代码）     |");

    println!("\n===== 4. &dyn Trait vs Box<dyn Trait> =====\n");

    let btn = Button { label: String::from("引用按钮") };
    let txt = TextBox { text: String::from("引用文本") };

    // &dyn Trait：借用，不拿所有权
    let refs: Vec<&dyn Draw> = vec![&btn, &txt];
    for r in refs {
        r.draw();
    }

    // Box<dyn Trait>：拥有，堆上分配
    let owned: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: String::from("拥有按钮") }),
        Box::new(TextBox { text: String::from("拥有文本") }),
    ];
    for o in &owned {
        o.draw();
    }

    println!("\n===== 5. 什么时候用 trait 对象？=====\n");

    // ✅ 用 trait 对象的场景：
    // 1. 需要存储「不同类型」的集合（如 GUI 组件列表）
    // 2. 类型种类多或运行时才知道（如插件系统）
    // 3. 想减少二进制大小（避免单态化爆炸）
    //
    // ❌ 不用 trait 对象的场景：
    // 1. 类型固定且少（用泛型更高效）
    // 2. 性能极致敏感（动态分发有开销）
    // 3. trait 不是「对象安全」的（下一节讲）

    println!("✅ 用 dyn Trait：");
    println!("   - 存不同类型的集合（GUI 组件、插件）");
    println!("   - 类型运行时才知道");
    println!("   - 减少二进制大小");
    println!();
    println!("❌ 不用 dyn Trait：");
    println!("   - 类型固定且少（用泛型）");
    println!("   - 性能敏感（动态分发有开销）");
    println!("   - trait 不对象安全");
}
