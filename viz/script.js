
fetch("./da.txt")
.then(response => response.text())
.then(text => main(text))

function main(data){


    arr = data.split("\n")
    arr.shift()
 
    arr = arr.map((item)=>{
        return item.split(" ");
    });
    
    max_x = Math.max(...arr.map((item)=> item[0]));
    min_x = Math.min(...arr.map((item)=> item[0]));

    range_x = max_x - min_x;

    max_y = Math.max(...arr.map((item)=> item[1]));
    min_y = Math.min(...arr.map((item)=> item[1]));

    range_y = max_y - min_y

    normalized = arr.map((item)=> {
        return [(item[0]-min_x)/range_x ,(item[1]-min_y)/range_y ]
    } )

    const canvas = document.querySelector('#canvas');

    if (!canvas.getContext) {
        return;
    }

    const ctx = canvas.getContext('2d');

    var scale = 1;
    canvas.width = 1280 * scale;
    canvas.height = 739 * scale;

    drawPoints(normalized,ctx);

    fetch("./path.txt")
    .then(response => response.text())
    .then(path => {
        arr = path.split("\n")
        arr.shift()
        arr.push(0)
        drawLines(arr,ctx,"#1d8a3a");
    })

}


function drawLines(arr,ctx,color){
    for(let i =0; i<arr.length-1;i++){

        let x_start = canvas.width*normalized[arr[i]][0];
        let y_start = canvas.height*normalized[arr[i]][1];

        let x_end = canvas.width*normalized[arr[i+1]][0];
        let y_end = canvas.height*normalized[arr[i+1]][1];

        ctx.beginPath();
        ctx.moveTo(x_start, y_start);
        ctx.lineTo(x_end, y_end);
        ctx.strokeStyle = color;
        ctx.stroke();

    }
}


function drawPoints(normalized,ctx){
    normalized.forEach((x) =>{

        let X = canvas.width*x[0];
        let Y = canvas.height*x[1];
        let R = 4;

        

        ctx.beginPath();
        ctx.arc(X, Y, R, 0, 2 * Math.PI, false);
        ctx.lineWidth = 2;
        ctx.strokeStyle = '#FF0000';
        ctx.fillStyle = '#FF0000'
        ctx.fill();
        ctx.stroke();


    })
}



