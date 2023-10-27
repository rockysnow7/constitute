from flask import Flask, request
from llama_cpp import Llama, LlamaGrammar


grammar = LlamaGrammar.from_file("grammar.gbnf")
llm = Llama(
    model_path="models/nous-hermes-llama2-13b.Q5_K_M.gguf",
    n_gpu_layers=-1,
    n_ctx=2048,
)

app = Flask(__name__)

@app.route("/predict", methods=["POST"])
def predict():
    prompt = request.get_json()["prompt"]
    response = llm(
        prompt,
        grammar=grammar,
        max_tokens=-1,
    )["choices"][0]["text"].strip()
    print(f"{response=}")

    return response


if __name__ == "__main__":
    app.run(debug=True, host="127.0.0.1", port=5000)