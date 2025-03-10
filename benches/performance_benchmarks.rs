use criterion::{black_box, criterion_group, criterion_main, Criterion};
use screensage::{Conversation, MessageRole};

fn conversation_add_message_benchmark(c: &mut Criterion) {
    c.bench_function("add 100 messages", |b| {
        b.iter(|| {
            let mut conversation = Conversation::new("Benchmark", "test-model");
            for i in 0..100 {
                let role = if i % 2 == 0 { MessageRole::User } else { MessageRole::Assistant };
                conversation.add_message(role, &format!("Message {}", i));
            }
            black_box(conversation)
        })
    });
}

fn conversation_truncate_benchmark(c: &mut Criterion) {
    c.bench_function("truncate 1000 messages", |b| {
        b.iter_with_setup(
            || {
                let mut conversation = Conversation::new("Benchmark", "test-model");
                for i in 0..1000 {
                    let role = if i % 2 == 0 { MessageRole::User } else { MessageRole::Assistant };
                    conversation.add_message(role, &format!("Message {}", i));
                }
                conversation
            },
            |mut conversation| {
                black_box(conversation.truncate(100));
            }
        )
    });
}

criterion_group!(
    benches,
    conversation_add_message_benchmark,
    conversation_truncate_benchmark
);
criterion_main!(benches);
