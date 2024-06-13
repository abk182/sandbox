import React, { useEffect, useRef } from "react";

export const Image = ({
  height,
  width,
  data
}: {
  height: number;
  width: number;
  data: Uint8Array
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
            let pixels = img.data;
            let imageIndex = 0;

            for (var i = 0; i < data.length; i += 3) {
              pixels[imageIndex++] = +data[i]; // r
              pixels[imageIndex++] = +data[i + 1]; // g
              pixels[imageIndex++] = +data[i + 2]; // b
              pixels[imageIndex++] = +data[i + 3]; // a
            }
            ctx.putImageData(img, 0, 0);
        }

    }

  }, [height, width, data]);

  return <canvas ref={ref} />;
};
