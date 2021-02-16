import {add, sum_f64s} from './rust/src/lib.rs';

function addJs(n1, n2) {
  return add(n1, n2);
}

function sumDoubles(numbers) {
  return sum_f64s(numbers);
}

document.getElementById('result1').innerText = addJs(5, 14);

const numbers = new Float64Array([1.2, 2.3, 3.4, 4.5, 5.6, 6.7]);
console.log('numbers =', numbers);
const sum = sumDoubles(numbers);
console.log('sum =', sum);
document.getElementById('result2').innerText = sum;
