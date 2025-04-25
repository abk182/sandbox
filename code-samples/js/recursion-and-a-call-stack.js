const promblem = () => {
    function sum(n) {
        let result = 0;
    
        if (n > 0) {
            result = n + sum(n-1)
        }
    
        return result;
    }
    
    console.log(sum(1000000)); // max-call-stack-error;
}

const solutionWithMicrotaskQueue = () => {
    async function sum(n) {
        let result = 0;
    
        if (n > 0) {
            result = n + await Promise.resolve().then(() => sum(n - 1))
        }
    
        return result;
    }

    sum(10).then(console.log); // ??? works
}

const solutionWithTrampoline = () => {
    function trampoline(fn) {
        return function(...args) {
          let result = fn.apply(fn.context, args)
          while (typeof result === 'function') {
            result = result()
          }
      
          return result
        }
      }
      function sum(number, s = 0) {
        return number === 0 ? s : () => sum(number - 1, s + number)
      }
      const trampolineSum = trampoline(sum);
      console.log(trampolineSum(100000));
}
