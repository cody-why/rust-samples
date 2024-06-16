'''
Author: plucky
Date: 2022-07-05 09:27:01
LastEditTime: 2022-07-05 11:04:03
Description: 将excel文档转为csv文档
'''

# pip install pandas
# pig install openpyxl

from importlib.resources import path
import pandas
import os
 
#查找符合文件类型的文件
def file_name(file_dir,source_type):
    L=[]
    for root, dirs, files in os.walk(file_dir):
        for file in files:
            if os.path.splitext(file)[1] == source_type:
                L.append(os.path.splitext(file)[0])

    return L 

#将excel文档转为csv文档 
def excel_to_csv(file,to_file):
    #sheetname指定取哪个表, default 0,第一个表,返回多表使用sheetname=[0,1],若sheetname=None是返回全表
    #header指定取列名的行，default 0，第一行,若数据不含列名,则设定 header = None
    file_excel=pandas.read_excel(file,sheet_name=0)
    file_excel.to_csv(to_file,index=False)

if __name__=='__main__':
    #原文档所在目录
    source_path=os.path.abspath(os.path.dirname(__file__))
    print("目录: "+source_path)
    #转换文档存储目录
    object_path=source_path
    source_type='.xlsx'
    object_type='.csv' 
    
    file_list=file_name(source_path,source_type)
    for i in file_list:
        print("文件: "+i)
        file=source_path+'/'+i+source_type
        to_file=object_path+'/'+i+object_type
        excel_to_csv(file,to_file)
