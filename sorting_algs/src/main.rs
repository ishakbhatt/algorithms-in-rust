// Selection Sort

/*
 * Sorts an array by repeatedly finding the min element (considering ascending order)
 * from unsorted part and putting at the beginning. The algorithm maintains two subarrays:
 * 1) Sorted subarray
 * 2) Unsorted subarray
 * In every iteration of selection sort, min element from unsorted subarray is picked and moved to
 * sorted subarray.
 * Worst Case: O(n^2)
 * Avg Case: O(n^2)
 * Best Case: O(n^2)
 * */

fn selection_sort(arr: &mut[i32]){
    //let mut min_idx = 0;
    
    // Move boundary of unsorted array one by one
    for i in 0..arr.len()-1{
        
        // Find min element in unsorted array
        let mut _min_idx = i;
        
        for j in (i+1)..arr.len(){
            if arr[j] < arr[_min_idx] {
                _min_idx = j;
            }
        }
        // swap min from unsorted with boundary element
        arr.swap(_min_idx, i);
    }

}

// Bubble Sort
/*
 * Sorts by repeatedly swapping the adjacent elements if they are in the wrong order.
 * It does one whole pass at the end to ensure that no more swaps are necessary.
 * Worst Case: O(n^2)
 * Average Case: Theta(n^2)
 * Best Case: Omega(n)
 *
 * */

fn bubble_sort(arr: &mut[i32]){
    for i in 0..arr.len()-1{ // Go through entire array
        for j in 0..arr.len()-i-1{ // Elements before boundary in order
            if arr[j] > arr[j+1]{ // Check adjacent elements
                arr.swap(j, j+1);
            }
        }

    }

}

// Insertion Sort
/*
 * Array is split into sorted and unsorted parts. Values from unsorted part are picked and placed
 * at correct position in sorted part.
 *
 * STEPS:
 * 1) Iterate from arr[1] to arr[n]
 * 2) Compare current to predecessor
 * 3) If key element smaller than predecessor, compare to elements before. Move greater elements
 *    one position up to make space for swapped element.
 * Worst Case: O(n^2)
 * Average Case: Theta(n^2)
 * Best Case: Omega(n)
 *
 * */

fn insertion_sort(arr: &mut[i32]){
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i - 1; // index of elem before

        while j >= 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j = j - 1; // keep swapping with prior elements until sorted
        }
        arr[j+1] = key;
    }
}

// Merge Sort
/*
 * A divide and conquert algorithm that calls itself for the two halves, then merges the two
 * halves.
 *
 * Best Case: Omega(nlogn)
 * Avg Case: Theta(nlogn)
 * Worst CaseL O(nlogn)
 *
 *
 * */

fn merge(arr: &mut[i32], start:i32, mid:i32, end:i32){
    // Create copies of subarrays: L <-- arr[start..mid], H <-- arr[mid+1]
    let L: [i32, mid-start+1] = [0; mid-start+1];
    let H: [i32, end-mid] = [0; end-mid];

    for i in 0..mid+start-1 {
        L[i] = arr[i];
    } for j in 0..end-mid {
        H[j] = arr[mid + 1 + j];
    } 

    // Create pointers that maintain current indices of three arrays
    let mut curr_L = 0;
    let mut curr_H = 0;
    let mut curr_arr = start;

    // Pick larger among elements and place in correct position until reaching end of either
    while curr_L < (mid-start + 1) && curr_H < (end-mid){
        if L[curr_L] <= H[curr_H] {
            arr[curr_arr] = L[curr_L];
            curr_L++;
        } else {
            arr[curr_arr] = H[curr_H];
            curr_H++;
        }
        ++curr_arr;
    }

    // When you run out of either, append other subarray at the end of arr
    while curr_L < (mid-start+1) {
            arr[curr_arr] = L[curr_L];
            curr_arr++;
            curr_L;
    }
    while curr_H < (end-mid) {
            arr[curr_arr] = H[curr_H];
            curr_arr++;
            curr_H++;
        }
    }

}

fn merge_sort(arr: &mut[i32], start:i32, end:i32){
    let mid = ((start + end) / 2) - 1;
    merge_sort(arr, start, mid);
    merge_sort(arr, mid+1, end);
    merge(arr, start, mid, end);
}


// Quick Sort
/*
 * Take last element as pivot, places pivot element at correct position in sorted array, place all
 * smaller elements than pivot to left of pivot and greater elements to right of pivot.
 * 
 * Best Case: Omega(nlogn)
 * Avg Case: Theta(nlogn)
 * Worst Case: O(n^2)
 *
 *
 * */
fn partition(arr: &mut[i32], start:i32, end:i32){
    // select pivot
    let pivot:i32 = arr[end];

    // index of smaller element and indicates right position of pivot found so far
    int i = (start - 1);

    for j in start..end-1 {
        if arr[j] < pivot {
            i++; // increment index of smaller element
            arr.swap(i, j)
        }
    }
    arr.swap(i+1, end);
}


fn quick_sort(arr: &mut[i32], start:i32, end:i32){
    if(start < end){

        // part is partition index
        let part = partition(arr, start, end);

        // QuickSort before partition and after partition
        quickSort(arr, start, part-1);
        quickSort(arr, part+1, end);
    }
}

// Radix Sort
/*
* Digit by digit sort starting from LSD to MSD.
* Radix sort uses counting sort as a subroutine to sort.
* Lowest bound for comparison-based sorting is nlog(n).
* Counting sort is linear when elements are in range 1 to k.
* However, counting sort's worst case runtime is O(n^2).
* With Radix Sort you can sort 1 to k^2 in linear time.
*
*/

fn radixSort(arr: &mut[i32], n:i32){
	// Find max number to know max # of digits
	let mut max = arr[0];
	for i in 1..n {
		if arr[i] > m {
			max = arr[i];
		}
	}

	// Do counting sort for every digit. Start with LSD (1) 
	// and go to MSD (10^i based on max)
	let mut exp = 1;
	while exp / max > 0 {
		countSort(arr, n, exp);
		exp *= 10; // go up sig digits
	}

}
// Radix Sort Helper Fn: Counting Sort
fn countSort(arr: &mut[i32], n:i32, exp:i32){
	let output: [i32, n] = [0, n-1];
	let count: [i32, 10] = [0, 0];

	// Store number of occurrences of digits (0-9) in count bucket
	for i in 0..n {
		count[(arr[i] / exp) % 10]++;
	}

	// Change count[i] so that count[i] contains actual 
	// position of digit in output[i]
	for i in 1..10 {
		count[i] += count[i - 1];
	}

	// Build output array
	let mut i = n-1;
	while i >= 0 {
		output[count[(arr[i] / exp) % 10] - 1] = arr[i];
		count[(arr[i] / exp) % 10]--;
		i--;
	}

	// Copy output array to arr[]
	for i in 0..n {
		arr[i] = output[i];
	}

}

/////////////////////////////////////////////////////////
fn print_array(arr: &[i32]){
    for i in 0..arr.len() {
        print!(" {} ", arr[i]);
    }
    println!("");
}

fn main(){
    // declare array
    let mut array: [i32; 6] = [7, 3, 4, 5, 1, 6];
    
    // Original Array
    print!("Original Array: ");
    print_array(&array);

    // Sort using given algorithm
    //selection_sort(&mut array);
    //bubble_sort(&mut array);
    //insertion_sort(&mut array);
    //merge_sort(&mut array, 0, arr.len()-1);
    //quick_sort(&mut_array, 0, arr.len()-1);

    // Print after sort
    println!("After Selection Sort:");
    print_array(&array);

    // Fin
    println!("Success!");
}
