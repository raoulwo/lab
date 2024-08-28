// You can specify default values for type params.
type MyEvent0<T = HTMLElement> = {
  target: T,
  type: string
};

// The same thing goes for bounded generic type params.
type MyEvent1<T extends HTMLElement = HTMLElement> = {
  target: T,
  type: string
};

// NOTE: Like default function params, default generic type
// params must appear **after** all required params.
