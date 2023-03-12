type PortFromElm<T> = {
  subscribe: (callback: (fromElm: T) => void) => void;
};

type PortToElm<T> = {
  send: (dataSentToElm: T) => void;
};

export namespace Elm {
  namespace Main {
    type App = {
      ports: {
        greet: PortFromElm<string>;
        messageReceiver: PortToElm<string>;
      };
    };

    function init(args: { node: HTMLElement }): App;
  }
}
