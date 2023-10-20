import 'package:frontend/generated/payment.pbgrpc.dart';
import 'package:grpc/grpc.dart';

Future<void> main(List<String> arguments) async {
  final channel = ClientChannel('127.0.0.1',
      port: 8080,
      options: ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ));

  final client = PaymentsClient(channel);

  await client.initiatePayment(PaymentRequest(
    amount: '2000',
    from: 'Parth',
    id: 2232,
    reference: '33edsf',
    to: 'Milan',
  ));

  channel.shutdown();
}
