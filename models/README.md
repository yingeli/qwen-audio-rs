git lfs install

export MODEL_PATH="huggingface/Qwen2-Audio-7B-Instruct"
export ENGINE_DIR="engines/qwen2audio"
export CHECKPOINT_DIR="checkpoints/qwen2audio_1gpu_bf16"

git clone https://huggingface.co/Qwen/Qwen2-Audio-7B-Instruct $MODEL_PATH

python3 /app/tensorrt_llm/examples/models/core/multimodal/build_multimodal_engine.py \
    --model_type qwen2_audio \
    --model_path $MODEL_PATH \
    --max_batch_size 8 \
    --output_dir ${ENGINE_DIR}_b8/audio

python3 /app/tensorrt_llm/examples/models/core/qwen/convert_checkpoint.py \
    --model_dir=$MODEL_PATH \
    --output_dir=$CHECKPOINT_DIR

NOTE: max_prompt_embedding_table_size = query_token_num * max_batch_size, therefore, if you change max_batch_size, --max_prompt_embedding_table_size must be reset accordingly.

trtllm-build --checkpoint_dir=$CHECKPOINT_DIR \
    --max_batch_size=32 \
    --max_prompt_embedding_table_size=131072 \
    --output_dir=${ENGINE_DIR}_b32/llm

python3 /app/tensorrt_llm/examples/models/core/qwen2audio/run.py \
    --tokenizer_dir=$MODEL_PATH \
    --engine_dir=${ENGINE_DIR}_b32/llm \
    --audio_engine_path=${ENGINE_DIR}_b32/audio/model.engine \
    --audio_url='../audio/glass-breaking-151256.mp3'

python3 run.py \
    --tokenizer_dir=$MODEL_PATH \
    --engine_dir=${ENGINE_DIR}_b32/llm \
    --audio_engine_path=${ENGINE_DIR}_b32/audio/model.engine \
    --audio_url='../audio/glass-breaking-151256.mp3'

python3 run_chat.py \
    --tokenizer_dir=$MODEL_PATH \
    --engine_dir=${ENGINE_DIR}_b32/llm \
    --audio_engine_path=${ENGINE_DIR}_b32/audio/model.engine \
    --max_new_tokens=256