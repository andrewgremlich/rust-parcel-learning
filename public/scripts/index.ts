const outputDiv = document.getElementById("output");
const runPrimeNumbers = document.getElementById("runPrimeNumbers");
const primeNumbersInput: HTMLInputElement | null = document.getElementById(
  "primeNumbersInput"
) as HTMLInputElement;

function isPrime(number) {
  let s = Math.sqrt(number);

  for (let i = 2; i <= s; i++) {
    if (number % i === 0) return false;
  }

  return number > 1;
}

runPrimeNumbers?.addEventListener("click", () => {
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
