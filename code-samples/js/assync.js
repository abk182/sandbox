const assync = async () => {
  console.log("ass begin");
  await new Promise((resolve) => {
    setTimeout(() => {
      resolve();
    }, 3000);
  });
  console.log("ass end");
};
const fn1 = () => {
  console.log("fn1 start");
  assync();
  console.log("fn1 end");
};
fn1();
const fn2 = async () => {
  console.log("fn2 start");
  await assync();
  console.log("fn2 end");
};
fn2();
const fn3 = async () => {
  console.log("fn3 start");
  await fn1();
  console.log("fn3 end");
};
fn3();
const fn4 = () => {
  console.log("fn4 start");
  return assync();
  console.log("fn4 end");
};
fn4();
const fn5 = async () => {
  console.log("fn5 start");
  await fn4();
  console.log("fn5 end");
};
fn5();
