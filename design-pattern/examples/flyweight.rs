/*
 * @Author: plucky
 * @Date: 2023-05-26 01:30:02
 * @LastEditTime: 2023-05-26 01:47:55
 * @Description: 
 */

// 享元模式
// 享元模式是一种设计模式，用于减少创建对象的数量，从而减少内存使用和提高性能。
#![allow(unused)]

use std::collections::HashMap;
use std::rc::Rc;
 struct Glyph {
    font: Rc<String>,
    size: u32
}
 struct GlyphFactory {
    cache: HashMap<(char, Rc<String>), Glyph>,
}
 impl GlyphFactory {
    fn create_glyph(&mut self, c: char, font: &Rc<String>, size: u32) -> &Glyph {
        let key = (c, font.clone());
        let key2 = key.clone();
        if !self.cache.contains_key(&key) {
            let glyph = Glyph{font: font.clone(), size: size};
            self.cache.insert(key, glyph);
        }
        self.cache.get(&key2).unwrap()
    }
}

// impl GlyphFactory {
//     fn create_glyph(&self, c: char, font: &Rc<String>, size: u32) -> &Glyph {
//         let key = (c, font.clone());
//         if !self.cache.contains_key(&key) {
//             let glyph = Glyph{font: font.clone(), size: size};
//             self.cache.insert(key, glyph);
//         }
//         self.cache.get(&key).unwrap()
//     }
// }

fn main() {
    let font = Rc::new(String::from("Arial"));
    let mut glyph_factory = GlyphFactory{cache: HashMap::new()};
    {
        let g1 = glyph_factory.create_glyph('A', &font, 12);
        assert!(g1.font.eq(&font));
    }
    {
        let g2 = glyph_factory.create_glyph('A', &font, 14);
        assert!(g2.font.eq(&font));
    }
    let g3 = glyph_factory.create_glyph('B', &font, 12);
    assert!(g3.font.eq(&font));
}

// 在这个例子中，我们定义了一个 Glyph 结构体表示字符，包含了内部状态 font 和 size 。我们还定义了一个 GlyphFactory 结构体来共享内部状态，使用哈希表来缓存已经创建的字符对象。在 create_glyph 方法中，我们使用元组作为键来查找缓存中的对象，如果不存在则创建，并将其添加到缓存中。最后，我们使用 assert 语句来测试对象是否共享相同的内部状态。 
// 享元模式是Rust中常用的设计模式之一，可以有效地优化性能和内存消耗。由于Rust对内存处理的控制较好，使用该模式可以更好地发挥语言的优势。