run_python_server:
    source ~/llm_env/bin/activate &&cd timestone_python/src_python && uvicorn main:app --reload

build_python_bindings:
    cd timestone_python && maturin develop


