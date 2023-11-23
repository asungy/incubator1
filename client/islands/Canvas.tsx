import { useRef, useEffect } from "preact/hooks";

export default function Canvas() {
  const canvasRef = useRef(null);

  useEffect(() => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.fillStyle = "#f00";
    ctx.fillRect(0, 0, 120, 120);

    const imageData = ctx.createImageData(100, 100);
    for (let i = 0; i < imageData.data.length; i += 4) {
      imageData.data[i + 3] = 255;
    }
    console.log(imageData);
    ctx.putImageData(imageData, 0, 0);
  }, []);

  return (
    <>
      <h1>Hello, Canvas!</h1>
      <canvas style={{border: "1px solid black"}} ref={canvasRef} />
    </>
  );
}
