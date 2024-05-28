/*
 * @Date: 2024-05-15 19:44:03
 * @LastEditTime: 2024-05-15 20:30:22
 */

pub mod router;

// 定义上下文结构体，包含参数param和id
#[derive(Clone, Debug)]
pub struct Context {
    param: String,
    id: u32,
}

impl Context {
    pub fn new(param: String, id: u32) -> Context {
        Context { param, id }
    }
}

// 定义FromContext特质
pub trait FromContext {
    fn from_context(context: &Context) -> Self;
}

// 定义处理器特质, T泛型是指Handler的参数类型,call就是调用handler
pub trait Handler<T> {
    // 处理函数，接受上下文和处理参数
    fn call(self, context: Context);
}

// 定义Param结构体，包装String类型的参数
pub struct Param(pub String);

impl FromContext for Param {
    // 从上下文中创建Param实例
    fn from_context(context: &Context) -> Self {
        Param(context.param.clone())
    }
}

// 定义Id结构体，包装u32类型的参数
pub struct Id(pub u32);

impl FromContext for Id {
    // 从上下文中创建Id实例
    fn from_context(context: &Context) -> Self {
        Id(context.id)
    }
}

// 宏定义，生成处理器实现代码, name!是一个宏,实现了1个到多个参数的处理器
macro_rules! all_the_tuples {
    ($name:ident) => {
        $name!([]);
        $name!([T1]);
        $name!([T1, T2]);
        $name!([T1, T2, T3]);
        $name!([T1, T2, T3, T4]);
        // ... 更多
    };
}

// 宏定义，实现处理器特质
macro_rules! impl_handler {
    (
        // 宏参数，处理器参数类型列表
        [$($ty:ident),*]
    ) => {
        // 实现Handler特质
        impl<F, $($ty,)*> Handler<($($ty,)*)> for F
        where
            F: Fn($($ty,)*),
            $( $ty: FromContext,)*
        {
            // 处理函数，从上下文中获取参数并调用处理函数
            fn call(self, _context: Context) {
                self($( $ty::from_context(&_context), )*);
            }
        }
    };
}

// 生成处理器实现代码
all_the_tuples!(impl_handler);


/*
// 执行 impl<F, T> Handler<T> for F 后，相当于为 Fn<T> 类型实现了 Handler 这个 trait，即 print_id 实现了 Handler，可以调用 call 方法，而 call 方法中的 self 就是 print_id

impl<F, T> Handler<T> for F
where
    F: Fn(T),
    T: FromContext,
{
    fn call(self, context: Context) {
        self(T::from_context(&context));
    }
}

// 实现Handler特质, 2个参数的处理器
impl<T1, T2, F> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromContext,
    T2: FromContext,
{
    fn call(self, context: Context) {
        (self)(T1::from_context(&context), T2::from_context(&context));
    }
}

*/