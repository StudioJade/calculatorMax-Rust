print("正在安装外部库……")
import subprocess,sys
try:
    subprocess.run([sys.executable,"-m","pip", "install", "--upgrade", "pip"])
    subprocess.run([sys.executable,"-m","pip", "install", "easygui"])
    subprocess.run([sys.executable,'-m','pip','install','simpleeval'])
except Exception as e:
    print("外部库安装失败，请按照说明书手动安装\n"+e[0])
    sys.exit()
#运行这段程序需要使用外部库easygui和simpleeval，通过上述代码安装后可以使用
use_simple_eval=False
print('正在导入模块……')
try:
    import easygui
    from random import *
    from math import *  
    from simpleeval import simple_eval
except ImportError:
    print('导入失败！')
    sys.exit()
except:
    print('导入时发生未知错误！')
    sys.exit()
print('正在初始化……')
def useinfo():
    def s_tri(bot, high)->float:
        return bot*high/2
    def s_rect(bot, high):
        return bot*high
    def s_tra(bot,top, high)->float:
        return (bot+top)*high/2
    def hsf_s_tri(a,b,c)->float:
        s=(a+b+c)/2
        return sqrt(s*(s-a)*(s-b)*(s-c))
    def pt(a,b)->float:
        return sqrt(pow(a,2)+pow(b,2))
    def s_circle(r)->float:
        return 3.141592653589793238462643383279502884197169399875105923074944*r*r
    m=0
    history={}
    while True:
        c=easygui.buttonbox(title="calculatorMax",msg="calculatorMax，计算一切结果",choices=['开始计算','使用说明','历史记录','设置','退出'])
        if c=='开始计算':
            while True:
                f='未知错误'
                try:
                    ev=easygui.enterbox(msg='请输入算式',title='calculatorMax')
                    if use_simple_eval:
                        ev.replace("m","m()")
                        ev.replace("pi","pi()")
                        ev.replace("e","e()")
                        f=str(simple_eval(ev,
                        functions={"m":lambda: m,
                                   "pi":lambda: pi,
                                   "e":lambda: e,
                                   "pow":lambda a,b: pow(a,b),
                                "sqrt":lambda a: sqrt(a),
                                "sin":lambda a: sin(a),
                                "cos":lambda a: cos(a),
                                "tan":lambda a: tan(a),
                                "asin":lambda a: asin(a),
                                "acos":lambda a: acos(a),
                                "atan":lambda a: atan(a),
                                "log":lambda a: log(a),
                                "log10":lambda a: log10(a),
                                "log2":lambda a: log2(a),
                                "exp":lambda a: exp(a),
                                "sinh":lambda a: sinh(a),
                                "cosh":lambda a: cosh(a),
                                "tanh":lambda a: tanh(a),
                                "gamma":lambda a: gamma(a),
                                "erf":lambda a: erf(a),
                                "erfc":lambda a: erfc(a),
                                "ceil":lambda a: ceil(a),
                                "floor":lambda a: floor(a),
                                "trunc":lambda a: trunc(a),
                                "modf":lambda a: modf(a),
                                "fabs":lambda a: fabs(a),
                                "factorial":lambda a: factorial(a),
                                "isinf":lambda a: isinf(a),
                                "isnan":lambda a: isnan(a),
                                "isclose":lambda a, b: isclose(a,b),
                                "gcd":lambda a, b: gcd(a,b),
                                "lcm":lambda a, b: lcm(a,b),
                                "s_tri":lambda a, b: s_tri(a,b),
                                "s_rect":lambda a, b: s_rect(a,b),
                                "s_circle":lambda a: s_circle(a),
                                "s_tra":lambda a, b, c: s_tra(a,b,c),
                                "hsf_s_tri":lambda a, b, c: hsf_s_tri(a,b,c),
                                "pt":lambda a, b: pt(a,b),
                                "randint":lambda a, b: randint(a,b),
                                "random":lambda: random(),
                                "randrange":lambda a, b: randrange(a,b),
                                "uniform":lambda a, b: uniform(a,b)
                        }))
                    else:
                        f=str(eval(ev))
                    err=False
                except OverflowError:
                    f='浮点数溢出'
                    err=True
                except ZeroDivisionError:
                    f='除零'
                    err=True
                except FloatingPointError:
                    f='浮点数异常'
                    err=True
                except ValueError:
                    f='值错误'
                    err=True
                except TypeError:
                    f='类型错误'
                    err=True
                except:
                    err=True
                    try:
                        if isnan(f):
                            f='不是数字'
                        elif isinf(f):
                            f='溢出'
                        else:
                            f='未知错误'
                    except:
                        f='可能不是数学算式'
                history[ev]=f
                choices=['继续','返回首页','退出']
                if not err:
                    choices.append('记忆')
                c=easygui.buttonbox(title='结果-calculatorMax',msg=ev+'='+f, choices=choices)
                if c=='继续':
                    continue
                elif c=='返回首页':
                    break
                elif c=='退出':
                    if easygui.ynbox(title='calculatorMax',msg='确定退出？'):
                        sys.exit()
                elif c=='记忆':
                    m=f
        elif c=='使用说明':
            useinfo()
        elif c=='历史记录':
            hr_str=''
            for i in history:
                hr_str+=i+'='+history[i]+'\n'
            c=easygui.buttonbox(title='历史记录-calculatorMax',msg=hr_str,choices=['返回','存储'])
            if c=='存储':
                try:
                    f=open(easygui.enterbox(title='claculatorMax',msg='请输入存储文件路径，请确保该文件存在且不为空'),'w')
                    f.write(hr_str)
                    f.close()
                    easygui.msgbox(title='calculatorMax',msg='存储完成！')
                except FileNotFoundError:
                    easygui.msgbox(title='calculatorMax',msg='存储失败！\n原因：该路径不是一个文本文件或不存在。')
                except IOError:
                    easygui.msgbox(title='calculatorMax',msg='存储失败！\n原因：写文件时出错。')
                except:
                    easygui.msgbox(title='calculatorMax',msg='存储失败！\n原因：未知错误。')
                try:
                    f.close()
                finally:#不写会报错所以我就写了
                    pass
        elif c=='设置':
            while True:
                c=easygui.buttonbox(title='设置-calculatorMax',msg='设置',choices=['返回','清空历史记录','simpleeval设置'])
                if c=='清空历史记录' and easygui.ynbox('确定清空历史记录吗？','calculatorMax'):
                    history={}
                elif c=='返回':
                    break
                elif c=='simpleeval设置':
                    while True:
                        c=easygui.buttonbox(title='simpleeval设置-calculatorMax',msg='simpleeval外部库有类似eval的功能，可以“给字符串去掉引号”。但是它比普通eval()函数更加安全，只能执行指定的功能。',choices=['返回','simpleeval模式：'+{'True':'开','False':'关'}[str(use_simple_eval)]])
                        if c=='返回':
                            break
                        elif c=='simpleeval模式：'+{'True':'开','False':'关'}[str(use_simple_eval)]:
                            use_simple_eval=not use_simple_eval
        elif c=='退出':
            break
if easygui.ynbox(title='calculatorMax',msg='确定退出？'):
    sys.exit()
