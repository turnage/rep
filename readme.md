# rep

This is a personal challenge to search strings fast.

The benchmark script searches a small slice of an access log NASA (~10MB)
uploaded for the date stamp of the last line. I will sophisticate the benchmark
over time.

#### November 12th, 2016, 15:06 PT

I know nothing about string searching. I wrote a brute force n^2 comparison
loop.

````
real    0m1.688s
user    0m1.624s
sys     0m0.048s
````

#### November 12th, 2016 15:33 PT

I ditched iterators and cut ~500ms.

````
real    0m1.137s
user    0m1.024s
sys     0m0.088s
````

#### November 12th, 2016 16:28 PT

I implemented Boyer-Moore-Horspool and cut ~400ms.

````
real    0m0.754s
user    0m0.660s
sys     0m0.072s
````

#### November 12th, 2016 17:40 PT

I implemented Boyer-Moore and cut ~500ms.

````
real    0m0.209s
user    0m0.164s
sys     0m0.036s
````
