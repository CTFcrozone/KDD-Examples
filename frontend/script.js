window.addEventListener("DOMContentLoaded", () => {

// change the IP to minikube external IP
  const sse = new EventSource("http://IP:8081/sse");
  const div = document.querySelector("#sse");

  sse.addEventListener("sse-event", (e) => {
    let p = document.createElement("p");

    p.textContent = e.data;

    div.appendChild(p);
  });
});
