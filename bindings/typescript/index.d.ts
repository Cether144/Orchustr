export class PromptBuilder {
  template(value: string): PromptBuilder;
  build(): { render(context: Record<string, unknown>): string };
}

export class GraphBuilder<T extends Record<string, unknown>> {
  /**
   * Note: The generic type `T` provides compile-time safety in TypeScript but
   * is not enforced at runtime in JavaScript.  Node handler return types are
   * not validated — runtime mismatches will surface as plain JS errors.
   */
  addNode(name: string, handler: (state: T) => Promise<T> | T): GraphBuilder<T>;
  addEdge(source: string, target: string): GraphBuilder<T>;
  setEntry(name: string): GraphBuilder<T>;
  setExit(name: string): GraphBuilder<T>;
  build(): { execute(state: T): Promise<T> };
}

export class ForgeRegistry {
  register(name: string, handler: (args: Record<string, unknown>) => Promise<unknown> | unknown): void;
  importFromMcp(client: NexusClient): Promise<number>;
  invoke(name: string, args: Record<string, unknown>): Promise<unknown>;
}

export class NexusClient {
  static connectHttp(endpoint: string): Promise<NexusClient>;
  send(method: string, params: Record<string, unknown>): Promise<any>;
  listTools(): Promise<Array<{ name: string }>>;
  invokeTool(name: string, args: Record<string, unknown>): Promise<any>;
}

export class OpenAiConduit {
  static fromEnv(): OpenAiConduit;
  completeText(prompt: string): Promise<{ text: string }>;
  completeMessages(messages: Array<Record<string, unknown>>): Promise<{ text: string }>;
  streamText(prompt: string): AsyncIterable<string>;
}

export class AnthropicConduit {
  static fromEnv(): AnthropicConduit;
  completeText(prompt: string): Promise<{ text: string }>;
  completeMessages(messages: Array<Record<string, unknown>>): Promise<{ text: string }>;
  streamText(prompt: string): AsyncIterable<string>;
}
