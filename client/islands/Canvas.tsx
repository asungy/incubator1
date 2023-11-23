import { useRef, useEffect, useState } from "preact/hooks";

class Data {
  width: number;
  height: number;
  toggle: boolean;

  constructor() {
    this.width = 100;
    this.height = 100;
    this.toggle = false;
  }
}

export default function Canvas() {
  const canvasRef = useRef(null);
  const [ data, setData ] = useState<Data>(new Data());

  useEffect(() => {
    const ctx = canvasRef.current.getContext("2d");
    const imageData = ctx.createImageData(data.width, data.height);
    if (data.toggle) {
      for (let i = 0; i < imageData.data.length; i += 4) {
        imageData.data[i + 0] = 255;
        imageData.data[i + 3] = 255;
      }
    } else {
      for (let i = 0; i < imageData.data.length; i += 4) {
        imageData.data[i + 1] = 255;
        imageData.data[i + 3] = 255;
      }
   }
    ctx.putImageData(imageData, 0, 0);

    setData({...data, toggle: !data.toggle})
  }, [data]);

  return (
    <>
      <h1>Hello, Canvas!</h1>
      <canvas style={{border: "1px solid black"}} ref={canvasRef} />
    </>
  );
}
