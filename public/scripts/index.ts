const outputDiv = document.getElementById("output");
const runPrimeNumbers = document.getElementById("runPrimeNumbers");
const primeNumbersInput: HTMLInputElement | null = document.getElementById(
  "primeNumbersInput"
) as HTMLInputElement;

function isPrime(number) {
  for (let i = 2, s = Math.sqrt(number); i <= s; i++)
    if (number % i === 0) return false;
  return number > 1;
}

runPrimeNumbers?.addEventListener("click", () => {
  console.log('hello world');
  if (primeNumbersInput?.value) {
    console.time("Web app prime number calc");

    const primes: number[] = [];

    for (let i = 2; i <= +primeNumbersInput?.value; i++) {
      if (isPrime(i)) {
        primes.push(i);
      }
    }

    if (outputDiv) {
      outputDiv.innerHTML = `<p>${JSON.stringify(primes)}</p>`;
      console.timeEnd("Web app prime number calc");
    }
  }
});
