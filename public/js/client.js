
const initContentDiv = () => {
    let contentDiv = document.querySelector('#content-div');
    if (contentDiv) {
        contentDiv.remove();
    }

    contentDiv = document.createElement('div');
    contentDiv.setAttribute('id', 'content-div');

    document.body.appendChild(contentDiv);
}


const generateTimeTable = (departures) => {

    initContentDiv();

    const contentDiv = document.querySelector('#content-div');

    if (contentDiv) {
        departures.forEach(departure => {
            const div = document.createElement('div');
            div.innerText = departure.name + ' | ' + departure.time;
            contentDiv.appendChild(div);
        });
    }

}


fetch('http://localhost:8000/api')
    .then(res => res.json())
    .then(data => {
        console.log(data);
        generateTimeTable(data.Departure)
    })