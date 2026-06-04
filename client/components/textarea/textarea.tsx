import { h, TextareaHTMLAttributes } from "preact";
import { useRef } from "preact/hooks";
import { classNames } from "../../kit/class-names.ts";

type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & {
  autoResize?: boolean;
};

export function Textarea({ autoResize, className, ...props }: TextareaProps) {
  const ref = useRef<HTMLTextAreaElement>(null);

  return (
    <textarea
      className={classNames(
        "component-textarea",
        { resize: autoResize },
        className,
      )}
      ref={ref}
      {...props}
    />
  );
}
