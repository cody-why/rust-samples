/*
 * @Author: plucky
 * @Date: 2023-05-24 21:06:09
 * @LastEditTime: 2023-05-24 22:08:58
 * @Description: 
 */
// 策略模式

trait Sorter<T> {
    fn sort(&self, slice: &mut [T]);
}
struct BubbleSorter;
impl<T: Ord> Sorter<T> for BubbleSorter {
    fn sort(&self, slice: &mut [T]) {
        for i in 0..slice.len() {
            for j in i + 1..slice.len() {
                if slice[j] < slice[i] {
                    slice.swap(i, j);
                }
            }
        }
    }
}
 struct QuickSorter;
impl<T: Ord> Sorter<T> for QuickSorter {
    fn sort(&self, slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }
        let pivot = partition(slice);
        let (left, right) = slice.split_at_mut(pivot);
        Sorter::<T>::sort(&QuickSorter, left);
        Sorter::<T>::sort(&QuickSorter, &mut right[1..]);
    }
}
 fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let last = slice.len() - 1;
    let pivot = (0..last).fold(0, |pivot, i| {
        if slice[i] < slice[last] {
            pivot + 1
        } else {
            pivot
        }
    });
    slice.swap(pivot, last);
    pivot
}

// 根据new的时候策略的不同，调用不同的排序算法
 struct SortContext<'a, T> {
    sorter: &'a dyn Sorter<T>,
}
 impl<'a, T> SortContext<'a, T> {
    fn new(sorter: &'a dyn Sorter<T>) -> Self {
        SortContext { sorter }
    }
    fn sort(&self, slice: &mut [T]) {
        self.sorter.sort(slice);
    }
}
 fn main() {
    let mut numbers = vec![1, 4, 3, 2, 5];
    let bubble_sorter = BubbleSorter {};
    let quick_sorter = QuickSorter {};
    let bubble_sort_context = SortContext::new(&bubble_sorter);
    let quick_sort_context = SortContext::new(&quick_sorter);
    bubble_sort_context.sort(&mut numbers);
    println!("After bubble sort: {:?}", numbers);
    let mut numbers = vec![1, 4, 3, 2, 5];
    quick_sort_context.sort(&mut numbers);
    println!("After quick sort: {:?}", numbers);
}

// 在此示例中，我们定义了一个具有排序方法的 Sorter 特征，该方法需要可变的元素切片进行排序。
// 我们还定义了Sorter的两个具体实现：BubbleSorter和QuickSorter。  
// 然后，我们定义一个 SortContext 结构，该结构封装了对 Sorter 具体实现的引用，并提供了一个排序方法来使用所选实现对元素切片进行排序。  
// 在主要情况下，我们使用BubbleSorter创建一个SortContext的实例，使用bubble_sort_context对数字切片进行排序，打印排序后的数字切片，然后使用QuickSorter创建SortContext的另一个实例，使用quick_sort_context对数字切片进行排序，然后再次打印排序的数字切片。  
// 这使我们能够轻松更改运行时使用的排序算法，而无需更改使用 SortContext 的客户端代码 .

