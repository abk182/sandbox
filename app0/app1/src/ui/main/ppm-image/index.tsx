import React, { useEffect, useRef } from "react";

export const PpmImage = ({
  height,
  width,
  data
}: {
  height: number;
  width: number;
  data: string
}) => {
  const ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = ref.current;
    if (canvas) {
        canvas.width = width;
        canvas.height = height;
        const ctx = canvas.getContext("2d");

        if (ctx) {
            ctx.clearRect(0, 0, canvas.width, canvas.height)
            let img = ctx.getImageData(0, 0, width, height);
            let d = data.split(/\s+/);
            let pixels = img.data;
            let imageIndex = 0;

            for (var i = 4; i < d.length; i += 3) {
              pixels[imageIndex++] = +d[i]; // r
              pixels[imageIndex++] = +d[i + 1]; // g
              pixels[imageIndex++] = +d[i + 2]; // b
              pixels[imageIndex++] = 255; // a
            }
            ctx.putImageData(img, 0, 0);
        }

    }

  }, [height, width]);

  return <canvas ref={ref} />;
};
