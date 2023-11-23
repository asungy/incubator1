import { useRef, useEffect } from "preact/hooks";

export default function Canvas() {
  const canvasRef = useRef(null);

  useEffect(() => {
    const ctx = canvasRef.current.getContext("2d");
    ctx.fillStyle = "#f00";
    ctx.fillRect(0, 0, 120, 120);
  }, []);

  return (
    <>
      <h1>Hello, Canvas!</h1>
      <canvas ref={canvasRef} />
    </>
  );
}
