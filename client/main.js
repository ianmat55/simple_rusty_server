document.getElementById("myForm").addEventListener("submit", (event) => {
  event.preventDefault(); // Prevents the default form submit action

  let input = document.getElementById("input");
  let inputValue = parseInt(input.value, 10);

  if (inputValue > 100 || inputValue < 0) {
    // handle error
    console.log("NAH");
  } else {
    // Create an object to hold the data
    const data = { data: inputValue };

    // Send a POST request
    fetch("/", {
      method: "POST", // Specify the method
      headers: {
        "Content-Type": "application/json", // Specify the content type
      },
      body: JSON.stringify(data), // Convert the JavaScript object to a JSON string
    })
      .then((response) => response.json())
      .then((data) => {
        // populate h1 tag with response
        console.log("Success:", data);
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  }

  input.value = "";
});
